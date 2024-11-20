# syntax=docker/dockerfile:1

# Use a specific Node.js version
ARG NODE_VERSION=16

################################################################################
# Base image for all stages
FROM node:${NODE_VERSION}-alpine as base

# Set the working directory inside the container
WORKDIR /usr/src/app


################################################################################
# Install dependencies
FROM base as deps

# Copy the package files
COPY package.json package-lock.json ./

# Install dependencies using npm, leveraging caching for faster builds
RUN --mount=type=cache,target=/root/.npm \
    npm ci


################################################################################
# Build the application
FROM deps as build

# Copy all necessary project files
COPY . .

# Run the build process
RUN npm run build


################################################################################
# Final stage: minimal runtime image
FROM nginx:alpine as final

# Copy the built application from the build stage
COPY --from=build /usr/src/app/dist /usr/share/nginx/html

# Expose the port that the application will run on
EXPOSE 80

# Start the Nginx server
CMD ["nginx", "-g", "daemon off;"]
