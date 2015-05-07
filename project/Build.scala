import BaseSettings.Version
import sbt._
import sbt.Keys._
import io.gatling.sbt.GatlingPlugin

object BuildSettings {

  /*
   * All sub modules will re-use this "projectSettings" below
   */
  val projectSettings = Defaults.defaultConfigs ++ Defaults.itSettings ++
    BaseSettings.baseSettings ++ BaseSettings.projectSettings ++
    PublishSettings.publishSettings ++
    Classpaths.ivyPublishSettings ++ Classpaths.jvmPublishSettings

  val rootSettings = Defaults.defaultConfigs ++ Seq(
      fork in Test := false,
      fork in IntegrationTest := false,
      parallelExecution in Test := false,
      publishLocal := {},
      publish := {},
      //publishArtifact := false,
      version in ThisBuild := Version.ThisVersion
  )
  /*
   * Create a Version String - Include the git version sha in the build version for repeatable historical builds.
   */
  def generateVersion(major: String, minor: String, snapshot: Boolean) = {

    val gitHeadCommitSha = settingKey[String]("current git commit SHA")
    val incremental = Process("git rev-parse HEAD").lines.head
    s"$major.$minor-$incremental${if (snapshot) "-SNAPSHOT"}"
  }

}

/**
 * The SBT Build Object
 */
object RootBuild extends Build {

  import Dependencies._
  import BuildSettings._

  lazy val dadagenRoot = (project in file("."))
    .settings(rootSettings: _*) //.aggregate(dadagenCore)
    .settings(projectSettings: _*)
    .enablePlugins(GatlingPlugin)
    .settings(libraryDependencies ++=
        compile(
          generex
    //    commons.beanutils
          ,jackson.dfcsv
          ,scalastm
          ,logging.slf4j
          ,typesafeconfig
        )
        ++ test (
           testing.scalatest
          ,testing.scalacheck
          ,testing.scalamock
          ,gatling.testFramework // we are using gatling in a test to show how to use our library in gatling
          ,gatling.highcharts
          ,akka // using akka to test feeding an actorSystem with messages
        )
        ++ runtime (
          logging.logback
        )
     )

}
