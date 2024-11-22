# syntax=docker/dockerfile:1

ARG NODE_VERSION=18

################################################################################
# Base Stage: Common settings for all stages.
FROM node:${NODE_VERSION}-alpine AS base

# Set the working directory for all build stages.
WORKDIR /usr/src/app

COPY package*.json ./

RUN npm install

################################################################################
# Dependencies Stage: Install production dependencies.
FROM base AS deps

# Install production dependencies, leveraging caching for faster builds.

RUN --mount=type=cache,target=/root/.npm \
    npm ci --omit=dev

################################################################################
# Build Stage: Install dev dependencies and build the application.
FROM deps AS build

# Install all dependencies, including dev dependencies, for the build process.
RUN --mount=type=cache,target=/root/.npm \
    npm ci --omit=dev

# Copy the source files and build the application.
COPY . .

RUN npm run build

COPY . .

################################################################################
# Final Stage: Minimal runtime image.
FROM base AS final

# Use a non-root user for security.
USER node

# Copy runtime dependencies and the built application.
COPY --from=deps /usr/src/app/node_modules ./node_modules
#COPY --from=build /usr/src/app/dist ./dist

# Expose the application port.
EXPOSE 8000

# Start the application (adjust as needed).
CMD ["npm", "run", "dev"]
