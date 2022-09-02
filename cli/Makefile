
DOCKER_IMAGE:="ghcr.io/meta-secret/cli:latest"

docker_build:
	docker build -t ${DOCKER_IMAGE} .

docker_push: docker_build
	docker push ${DOCKER_IMAGE}

docker_run:
	docker run -ti --rm ${DOCKER_IMAGE}

docker_run_shell:
	docker run -ti --rm ${DOCKER_IMAGE} bash