FROM ubuntu:19.04

RUN apt-get update
RUN apt-get install -y curl lib32gcc1

WORKDIR /opt/steamcmd
RUN curl -sqL "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz" | tar zxvf -
RUN ./steamcmd.sh +quit # self update
RUN ./steamcmd.sh +login anonymous +force_install_dir /opt/srcds +app_update 232330 validate +quit

WORKDIR /opt/srcds
CMD ["./srcds_run", "-game", "cstrike", "+maxplayers 4", "+map de_dust2", "-nomaster", "+rcon_password", "12345"]
