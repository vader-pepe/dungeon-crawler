ARG CROSS_BASE_IMAGE

FROM $CROSS_BASE_IMAGE

COPY install_deb_bookworm.sh /
# Change the packages to your dependencies.
RUN chmod +x install_deb_bookworm.sh
RUN /install_deb_bookworm.sh amd64 libclang-11-dev \
  clang-11 \
  emscripten

# Update any environment variables required with `ENV`.
# ENV MYVAR=MYVALUE
