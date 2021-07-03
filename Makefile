.PHONY: help
.DEFAULT_GOAL := help
BUILD_DIR?=build
BUILD_TMP_DIR=${BUILD_DIR}/temp
BUILD_DEP_DIR=${BUILD_DIR}/dependencies

create_build_area:  ## Creates a build area local in the current directory
	@echo "Creating Build Area"
	@mkdir -p  $(CURDIR)/${BUILD_DIR}
	@mkdir -p  $(CURDIR)/${BUILD_TMP_DIR}
	@mkdir -p  $(CURDIR)/${BUILD_DEP_DIR}

setup_jvm: create_build_area ## Setup a Lucene compatible JVM. Currently JAava 11 LTS
	@echo "Downloading and setting up JDK 11"
	@curl -L "https://github.com/AdoptOpenJDK/openjdk11-binaries/releases/download/jdk-11.0.11%2B9_openj9-0.26.0/OpenJDK11U-jdk_x64_mac_openj9_11.0.11_9_openj9-0.26.0.tar.gz" --output  ${BUILD_DEP_DIR}/java.tar.gz
	@tar -zxvf ${BUILD_DEP_DIR}/java.tar.gz  -C  ${BUILD_DEP_DIR}
	@rm  ${BUILD_DEP_DIR}/java.tar.gz
	@mv ${BUILD_DEP_DIR}/jdk*  ${BUILD_DEP_DIR}/jdk

clean: ## cleanup all build artifacts
	@rm -rf  $(CURDIR)/${BUILD_DIR}

help:  ## Output help documentation
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

