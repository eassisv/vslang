FROM node:18-alpine
WORKDIR /app
COPY . .
RUN yarn
RUN yarn build
CMD yarn start 3000
