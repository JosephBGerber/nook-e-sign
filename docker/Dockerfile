FROM debian:buster-slim
ARG USER_ID

ENV WORKSPACE /home/developer/workspace
ENV KEYSTORE /home/developer/android-keystore.jks

RUN apt-get update && apt-get install -y \
	sudo \
	wget \
	unzip \
	lib32stdc++6 \
	lib32z1

# create developer user with user id of current user
RUN mkdir -p /home/developer/workspace && \
	useradd -u $USER_ID developer && \
	usermod -aG sudo developer && \
	usermod --shell /bin/bash developer && \
	chown developer:developer -R /home/developer && \
	echo "developer ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# install OpenJDK 8
ADD OpenJDK8U-jdk_x64_linux_hotspot_8u282b08.tar.gz /opt
ENV PATH="/opt/jdk8u282-b08/bin:${PATH}"

# download and install Android SDK
# https://developer.android.com/studio#command-tools
ENV ANDROID_SDK_ROOT /opt/android-sdk
RUN mkdir -p ${ANDROID_SDK_ROOT}/cmdline-tools && \
    wget -q https://dl.google.com/android/repository/commandlinetools-linux-6858069_latest.zip && \
    unzip *tools*linux*.zip -d ${ANDROID_SDK_ROOT}/cmdline-tools && \
    mv ${ANDROID_SDK_ROOT}/cmdline-tools/cmdline-tools ${ANDROID_SDK_ROOT}/cmdline-tools/tools && \
    rm *tools*linux*.zip
ENV PATH="/opt/android-sdk/platform-tools:${PATH}"

# install the necessary build tools
RUN yes | /opt/android-sdk/cmdline-tools/tools/bin/sdkmanager "platforms;android-7" "build-tools;19.1.0" "build-tools;29.0.2"

# add build scripts
ADD scripts /opt/scripts

ENV PATH="/opt/scripts:${PATH}"

# add keystore
ADD android-keystore.jks /home/developer/android-keystore.jks

# update PATH
# RUN echo PATH="/opt/scripts:/opt/android-sdk/platform-tools:/usr/local/openjdk-8/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" > /etc/environment

CMD ["/bin/su", "developer"]
