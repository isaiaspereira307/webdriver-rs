# Usar uma imagem base
FROM debian:latest

# Evitar perguntas durante a instalação de pacotes
ENV DEBIAN_FRONTEND=noninteractive

# Atualizar a lista de pacotes
RUN apt-get update

# Instalar ferramentas necessárias
RUN apt-get install -y wget tar

# Baixar o geckodriver
RUN wget -c https://github.com/mozilla/geckodriver/releases/download/v0.33.0/geckodriver-v0.33.0-linux64.tar.gz

# Desempacotar o geckodriver
RUN tar -xvzf geckodriver-v0.33.0-linux64.tar.gz

# Mover o geckodriver para o diretório de binários
RUN mv geckodriver /usr/bin/

# Remover o arquivo tar.gz
RUN rm geckodriver-v0.33.0-linux64.tar.gz

# Comando para executar o geckodriver em segundo plano e depois executar o outro executável
CMD /usr/bin/geckodriver & /path/to/your/executable

