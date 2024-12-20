# Base Python image
FROM python:3.10-slim AS base

# Set the working directory
WORKDIR /backend

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libpq-dev \
    curl && \
    curl -sSL https://install.python-poetry.org | python3 - && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set environment variables
ENV PATH="/root/.local/bin:$PATH" \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    POETRY_VERSION=1.7.0 \
    PYTHONPATH="/backend"



# Copy dependency files
COPY poetry.lock pyproject.toml ./

# Install dependencies (including dev dependencies for local development)
RUN poetry config virtualenvs.create false && \
    poetry install

# Copy the application
COPY ./app ./app
COPY alembic.ini .
COPY prestart.sh .
#COPY .env .

# Make the prestart script executable
RUN chmod +x prestart.sh

# Expose the application port
EXPOSE 8000

# Start the application with prestart script for production
CMD ["./prestart.sh"]

# Development stage
FROM base AS development
# Override CMD for development to enable hot reload
CMD ["poetry", "run", "uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000", "--reload"]

