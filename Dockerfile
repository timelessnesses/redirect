FROM python:3.13.0a3-slim

WORKDIR /app
COPY . /app

RUN pip install poetry
RUN poetry install

ENV DB_TYPE POSTGRES
ENV DB_HOST db
ENV DB_USER redirect
ENV DB_PASSWORD redirect
ENV DB_NAME redirect
ENV DB_PORT 5432

EXPOSE 8000 8000

CMD ["poetry", "run", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
