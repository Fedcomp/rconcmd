FROM ubuntu:16.04

RUN apt-get update
RUN apt-get install -y curl lib32gcc1

WORKDIR /opt/steamcmd
RUN curl -sqL "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz" | tar zxvf -
RUN ./steamcmd.sh +quit # self update
# RUN ./steamcmd.sh +login anonymous +force_install_dir /opt/hlds/ +app_update 90 +app_set_config 90 mod czero validate +quit
RUN ./steamcmd.sh +login anonymous +force_install_dir /opt/hlds +app_update 90 validate +quit
RUN ./steamcmd.sh +login anonymous +force_install_dir /opt/hlds +app_update 70 validate +quit || :
RUN ./steamcmd.sh +login anonymous +force_install_dir /opt/hlds +app_update 10 validate +quit || :
RUN ./steamcmd.sh +login anonymous +force_install_dir /opt/hlds +app_update 90 validate +quit

WORKDIR /opt/hlds
RUN echo "10" > steam_appid.txt
CMD ["./hlds_run", "-game", "cstrike", "+maxplayers 4", "+map de_dust2", "-nomaster", "+rcon_password", "12345"]
