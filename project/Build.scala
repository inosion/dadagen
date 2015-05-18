import BaseSettings.Version
import sbt._
import sbt.Keys._
import io.gatling.sbt.GatlingPlugin
import bintray.BintrayPlugin
import sbtassembly.AssemblyKeys._


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
      run in Compile <<= Defaults.runTask(fullClasspath in Compile, mainClass in (Compile, run), runner in (Compile, run)),
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
object DadagenBuild extends Build {

  import Dependencies._
  import BuildSettings._

  lazy val dadagenCore = (project in file("dadagen-core"))
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

  lazy val dadagenJmeter = Project( id="dadagen-jmeter", base = file("dadagen-jmeter"))
    .dependsOn(dadagenCore)
    .settings(projectSettings: _*)
    // frustrating!!! IntelliJ does not "see" a provided scope dependency, which is what JMeter needs to
    // be for dadgen plugin.
    .settings(libraryDependencies ++= compile(jmeter.core, scalalang.compiler))

    // really wish there was a cleaner way to do this
    .settings(assemblyExcludedJars in assembly := {
        val cp = (fullClasspath in assembly).value
        cp.filter {
          _.data.getName match {
            case x:String if x.contains("ApacheJMeter_core") => true
            case y:String if y.contains("jackson") => true
            case y:String if y.contains("jorphan") => true
            case y:String if y.contains("rsyntaxtextarea") => true
            case _ => false
          }
        }
     })


  lazy val dadagenRoot = (project in file("."))
    .settings(rootSettings: _*)
    .aggregate(dadagenCore, dadagenJmeter)

}
