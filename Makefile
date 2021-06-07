sbt_version   = $(shell cut -f2 -d= < project/build.properties)
scala_version = $(shell awk '/val Scala =/ { gsub(/"/, "", $$4); print $$4 }' < project/BaseSettings.scala)

gradle   := docker run --rm -u gradle -v ${HOME}/.gradle:/home/gradle/.gradle -v ${PWD}:/home/gradle/project -w /home/gradle/project gradle:6 gradle

.PHONY: build test deploy
build:
	$(gradle) build

test:
	$(gradle) test

deploy:
	$(gradle) release