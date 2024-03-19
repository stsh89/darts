FROM ruby:3.3.0
WORKDIR /app
COPY . .
RUN apt update && apt install -y build-essential
RUN bundle config set deployment 'true'
RUN bundle install --without test development
EXPOSE 3000
