# Use Node.js Alpine as base image for smaller size
FROM node:20-alpine

# Create a non-root user for security
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy application files
COPY . .

# Set proper permissions
RUN chown -R appuser:appgroup /app
USER appuser

# Make the script executable
RUN chmod +x index.js

# Set the entrypoint
ENTRYPOINT ["node", "index.js"]