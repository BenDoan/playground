FROM node
ADD . fertilizer
RUN cd /fertilizer && yarn && yarn build

FROM nginx
COPY --from=0 /fertilizer/dist /usr/share/nginx/html
