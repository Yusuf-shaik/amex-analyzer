FROM node:latest

WORKDIR /app

COPY ./*.json .
COPY ./*.js .

RUN npm install

COPY ./src ./src

EXPOSE 8081

CMD ["npm", "run", "serve"]
