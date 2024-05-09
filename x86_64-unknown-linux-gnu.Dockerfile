ARG CROSS_BASE_IMAGE

FROM $CROSS_BASE_IMAGE

COPY install_deb.sh /
# Change the packages to your dependencies.
RUN chmod +x install_deb.sh
RUN /install_deb.sh amd64 libclang-11-dev \
  clang-11 \
  xorg-dev

# Update any environment variables required with `ENV`.
# ENV MYVAR=MYVALUE
