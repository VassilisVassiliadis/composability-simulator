FROM ubuntu:bionic

RUN apt-get update && \
    export DEBIAN_FRONTEND=noninteractive && \
    apt-get install -y \
       python3.7 python3-pip python3-tk git locales curl libffi-dev libssl-dev \
       libpng-dev libjpeg-dev libfreetype6-dev pkg-config libxml2-dev libxslt-dev libpython3.7-dev
ENV LANGUAGE=en
ENV LC_ALL en_GB.UTF-8
ENV LANG en_GB.UTF-8

RUN locale-gen ${LC_ALL}

RUN python3 -m pip install --upgrade pip && \
    python3 -m pip install gsutil

SHELL ["/bin/bash", "-c"]
# VV: This expects that you've mounted `/output`
RUN echo $'#!/usr/bin/env bash\n\
    gsutil ls gs://clusterdata-2011-2/ && \n\
    gsutil -m cp -r gs://clusterdata-2011-2/task_events/ /output/task_events && \n\
    gsutil -m cp -r gs://clusterdata-2011-2/machine_events/ /output/machine_events && \n\
    gsutil cp gs://clusterdata-2011-2/schema.csv /output/schema.csv' >/download.sh && \
    chmod +x /download.sh

CMD /download.sh

# Other files/"directories":
# gs://clusterdata-2011-2/MD5SUM
# gs://clusterdata-2011-2/README
# gs://clusterdata-2011-2/SHA1SUM
# gs://clusterdata-2011-2/SHA256SUM
# gs://clusterdata-2011-2/schema.csv
# gs://clusterdata-2011-2/job_events/
# gs://clusterdata-2011-2/machine_attributes/
# gs://clusterdata-2011-2/machine_events/
# gs://clusterdata-2011-2/task_constraints/
# gs://clusterdata-2011-2/task_events/
# gs://clusterdata-2011-2/task_usage/

