FROM ubuntu:19.04

ARG SRCDS_DIR=/opt/srcds
ARG STEAMCMD_DIR=/opt/steamcmd

RUN apt-get update
RUN apt-get install -y curl lib32gcc1

RUN useradd srcds -m -d $SRCDS_DIR
RUN mkdir -p $STEAMCMD_DIR && chown -R srcds:srcds $STEAMCMD_DIR
USER srcds

WORKDIR $STEAMCMD_DIR
RUN curl -sqL "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz" | tar zxvf -
RUN ./steamcmd.sh +quit # self update
RUN ./steamcmd.sh +login anonymous +force_install_dir $SRCDS_DIR +app_update 232330 validate +quit

WORKDIR $SRCDS_DIR
CMD ["./srcds_run", "-game", "cstrike", "+maxplayers 4", "+map de_dust2", "-nomaster", "+rcon_password", "12345"]
