FROM alpine:3.18
ENV TZ=Europe/Madrid
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apk add --no-cache tzdata 
RUN apk add --update --no-cache netcat-openbsd 
EXPOSE 4444
COPY entrypoint.sh /
CMD ["/entrypoint.sh"]
