# # syntax=docker/dockerfile:1

# ARG NODE_VERSION=16

# ################################################################################
# # Use Node.js image as the base for all stages.
# FROM node:${NODE_VERSION}-alpine as base

# # Set the working directory for all build stages.
# WORKDIR /usr/src/app


# ################################################################################
# # Stage for installing production dependencies.
# FROM base as deps

# # Install dependencies, leveraging caching for faster builds.
# RUN --mount=type=bind,source=package.json,target=package.json \
#     --mount=type=bind,source=package-lock.json,target=package-lock.json \
#     --mount=type=cache,target=/root/.npm \
#     npm ci --omit=dev


# ################################################################################
# # Stage for building the application.
# FROM deps as build

# # Install dev dependencies for the build process.
# RUN --mount=type=bind,source=package.json,target=package.json \
#     --mount=type=bind,source=package-lock.json,target=package-lock.json \
#     --mount=type=cache,target=/root/.npm \
#     npm ci

# # Copy the source files and build the application.
# COPY /usr/src/app/frontend .
# RUN npm run build

# # List the contents of the build directory for debugging
# RUN ls -la /usr/src/app


# ################################################################################
# # Final stage: minimal runtime image.
# FROM base as final

# # Set Node.js environment to production.
# ENV NODE_ENV=production

# # Run the application as a non-root user.
# USER node

# # Copy package.json to support runtime commands.
# COPY package.json .

# # Copy the production dependencies and built application.
# COPY --from=deps /usr/src/app/node_modules ./node_modules
# COPY --from=build /usr/src/app/frontend ./frontend

# # Expose the application port.
# EXPOSE 8080

# # Command to run the application.
# CMD ["npm", "start"]

# syntax=docker/dockerfile:1

ARG NODE_VERSION=16

################################################################################
# Use Node.js image as the base for all stages.
FROM node:${NODE_VERSION}-alpine AS base

# Set the working directory for all build stages.
WORKDIR /usr/src/app


################################################################################
# Stage for installing production dependencies.
FROM base AS deps

# Install dependencies, leveraging caching for faster builds.
RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=package-lock.json,target=package-lock.json \
    --mount=type=cache,target=/root/.npm \
    npm ci --omit=dev


################################################################################
# Stage for building the application.
FROM deps AS build

# Install dev dependencies for the build process.
RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=package-lock.json,target=package-lock.json \
    --mount=type=cache,target=/root/.npm \
    npm ci

# Copy the source files and build the application.
COPY . .
RUN npm run build

# List the contents of the build directory for debugging
RUN ls -la /usr/src/app
RUN ls -la /usr/src/app/dist


################################################################################
# Final stage: minimal runtime image.
FROM base AS final

# Set Node.js environment to production.
ENV NODE_ENV=production

# Run the application as a non-root user.
USER node

# Copy package.json to support runtime commands.
COPY package.json .

# Copy the production dependencies and built application.
COPY --from=deps /usr/src/app/node_modules ./node_modules
COPY --from=build /usr/src/app/dist ./frontend

# Expose the application port.
EXPOSE 8080

# Start the application
CMD ["npm", "start"]
