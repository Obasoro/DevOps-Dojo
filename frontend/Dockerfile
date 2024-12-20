# syntax=docker/dockerfile:1

# Use Node.js 20 as the base image
FROM node:20-alpine AS base

# Set working directory
WORKDIR /app

################################################################################
# Dependencies Stage: Install all dependencies
FROM base AS deps

# Copy package files
COPY package*.json ./

# Install all dependencies
RUN npm install

################################################################################
# Build Stage: Copy source files
FROM deps AS build

# Copy application files
COPY . .

################################################################################
# Final Stage: Setup runtime environment
FROM base AS final

# Copy all application files and dependencies
COPY --from=build /app .

# Create non-root user for security
RUN addgroup -g 1001 nodejs && \
    adduser -S nodejs -u 1001 -G nodejs && \
    chown -R nodejs:nodejs /app

USER nodejs

# Expose the Vite development server port
EXPOSE 5173

# Start the development server
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]

