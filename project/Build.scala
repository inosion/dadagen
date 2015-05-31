import BaseSettings.Version
import sbt._
import sbt.Keys._
import io.gatling.sbt.GatlingPlugin
import bintray.BintrayKeys._
import sbtassembly.AssemblyKeys._
//import com.typesafe.sbt.SbtNativePackager._


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
      bintrayOrganization :=  Some("inosion"),
      bintrayPackage := "org.inosion.dadagen",
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

  lazy val dadagenCore = Project( id="dadagen-core", base = file("dadagen-core"))
    .settings(projectSettings: _*)
    .enablePlugins(GatlingPlugin)
    .disablePlugins(sbtassembly.AssemblyPlugin)
    .settings(libraryDependencies ++=
        compile(
          generex
          ,scalacsv
          ,scalastm
          ,logging.slf4j
          ,typesafeconfig
        )
        ++ test (
           testing.scalatest
          ,testing.scalacheck
          ,jackson.dfcsv
          ,testing.scalamock
          ,gatling.testFramework // we are using gatling in a test to show how to use our library in gatling
          ,gatling.highcharts
          ,akka.actor // using akka to test feeding an actorSystem with messages
        )
        ++ runtime (
          logging.logback
        )
     )

  /*
 * This is the ScalaFX File Generator - Just a Standalone GUI to create Lots of Files.
 */
  lazy val dadagenSupport = Project( id="dadagen-support", base = file("dadagen-support"))
    .dependsOn(dadagenCore)
    .settings(projectSettings: _*)
    .disablePlugins(sbtassembly.AssemblyPlugin)
    .settings(libraryDependencies ++= compile(scalalang.compiler))

  /*
   * This is the ScalaFX File Generator - Just a Standalone GUI to create Lots of Files.
   */
  lazy val dadagenUi = Project( id="dadagen-ui", base = file("dadagen-ui"))
    .dependsOn(dadagenCore, dadagenSupport)
    .settings(projectSettings: _*)
    .settings(mainClass in assembly := Some("org.inosion.dadagen.ui.DadagenUi") )
    .settings(assemblyJarName in assembly := s"dadagen-ui-${BaseSettings.Version.ThisVersion}.jar")
    .settings(libraryDependencies ++= compile(
        scalafx,
        rsyntaxtextarea,
        jackson.dfcsv,
        jackson.dfxml,
        jackson.databind,
        jackson.scala,
        stax2woodstox,
        akka.actor
      )
    )
    // for Java 7 only
    .settings(
      unmanagedJars in Compile +=
        Attributed.blank(
          file(scala.util.Properties.javaHome) / "/lib/jfxrt.jar")
    )
    .settings(addArtifact(Artifact("dadagen-ui", "assembly"), sbtassembly.AssemblyKeys.assembly))

  /*
   * This is the Native JMeter Plugin
   */
  lazy val dadagenJmeter = Project( id="dadagen-jmeter", base = file("dadagen-jmeter"))
    .dependsOn(dadagenCore, dadagenSupport)
    .settings(projectSettings: _*)
    .settings(libraryDependencies ++= provided(jmeter.core))
    .settings(assemblyJarName in assembly := s"dadagen-jmeter-plugin-${BaseSettings.Version.ThisVersion}.jar")
    // this adding artifact is needed for pushing assembly artifacts to bintray
    //  .settings(addArtifact(Artifact("dadagen-jmeter", "assembly"), sbtassembly.AssemblyKeys.assembly))

  lazy val dadagenRoot = (project in file("."))
    .settings(rootSettings: _*)
    .aggregate(dadagenCore, dadagenJmeter, dadagenUi)

}
