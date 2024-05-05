ARG CROSS_BASE_IMAGE

FROM $CROSS_BASE_IMAGE

COPY install_deb.sh /
# Change the packages to your dependencies.
RUN chmod +x install_deb.sh
ARG CROSS_DEB_ARCH
RUN /install_deb.sh $CROSS_DEB_ARCH libclang-10-dev \
clang-10

# Update any environment variables required with `ENV`.
# ENV MYVAR=MYVALUE
