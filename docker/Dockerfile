FROM openjdk:8
ARG USER_ID

ENV WORKSPACE /home/developer/workspace
ENV KEYSTORE /home/developer/android-keystore.jks

RUN apt-get update && apt-get install -y \
	sudo \
	lib32stdc++6 \
	lib32z1

# created developer user with user id of current user
RUN mkdir -p /home/developer/workspace && \
	useradd -u $USER_ID developer && \
	usermod -aG sudo developer && \
	usermod --shell /bin/bash developer && \
	chown developer:developer -R /home/developer


# download and install Gradle
# https://services.gradle.org/distributions/
ARG GRADLE_VERSION=6.5
ARG GRADLE_DIST=bin
RUN cd /opt && \
    wget -q https://services.gradle.org/distributions/gradle-${GRADLE_VERSION}-${GRADLE_DIST}.zip && \
    unzip gradle*.zip && \
    rm gradle*.zip

# download and install Android SDK
# https://developer.android.com/studio#command-tools
ARG ANDROID_SDK_VERSION=6858069
ENV ANDROID_SDK_ROOT /opt/android-sdk
RUN mkdir -p ${ANDROID_SDK_ROOT}/cmdline-tools && \
    wget -q https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_SDK_VERSION}_latest.zip && \
    unzip *tools*linux*.zip -d ${ANDROID_SDK_ROOT}/cmdline-tools && \
    mv ${ANDROID_SDK_ROOT}/cmdline-tools/cmdline-tools ${ANDROID_SDK_ROOT}/cmdline-tools/tools && \
    rm *tools*linux*.zip

# install the necessary build tools
RUN yes | /opt/android-sdk/cmdline-tools/tools/bin/sdkmanager "platforms;android-7" "build-tools;19.1.0" "build-tools;29.0.2"

# add build scripts
ADD scripts /opt/scripts

# add keystore
ADD android-keystore.jks /home/developer/android-keystore.jks

# update PATH
RUN echo PATH="/opt/scripts:/opt/android-sdk/platform-tools:/usr/local/openjdk-8/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" > /etc/environment

CMD ["/bin/su", "developer"]
