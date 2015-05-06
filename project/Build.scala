import BaseSettings.Version
import sbt._
import sbt.Keys._

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

  lazy val root = (project in file(".")).settings(rootSettings: _*).aggregate(redcliffeGraphine, redcliffeCommon)

  lazy val redcliffeCommon = (project in file("redcliffe-common"))
    .settings(projectSettings: _*)
    .settings(libraryDependencies ++= compile(spray.can, spray.routing, iostraightescqrs.core ))

  lazy val redcliffeCGT = (project in file("redcliffe-cgt-calc")).
    settings(projectSettings: _*).
    dependsOn(redcliffeGraphine).
    settings(libraryDependencies ++=
      compile(spray.can, spray.routing, jsonSchema)
    )


  lazy val redcliffeSpray = (project in file("redcliffe-spray")).
    settings(projectSettings: _*).
    dependsOn(redcliffeGraphine).
    settings(libraryDependencies ++= compile(spray.can, spray.routing, iostraightescqrs.spraysupport ))


  lazy val redcliffeGraphine = (project in file("redcliffe-graphine"))
    .settings(projectSettings: _*)
    .settings(libraryDependencies ++=
    compile(iostraightescqrs.core, iostraightescqrs.scalaz, kryoAkka,
      subcut, joda.time, joda.convert, joda.money, jasypt, commonsEmail, scalalang.compiler, scalalang.reflect,logging.slf4j,
      akka.persistence_cassandra,
      camel.akkaCamel, camel.flatpack, camel.ftp, camel.http,
      servletSpec,
      camel.mail   ,
      jodReports   , jodConverter  ,
      relfections    ,
      metawidget.core  , metawidget.scala  , metawidget.json    , metawidget.beanValidation  , metawidget.annotation       ,
      validation                  ,
      kryoAkka,
      logging.slf4jJcl, commonsLang
    ) ++
      test(testing.scalatest, testing.scalamock, spray.testkit) ++
      runtime(logging.logback)//,
    )

//  lazy val redcliffePlay = (project in file("redcliffe-play")).
//    settings(commonSettings: _*).
//    dependsOn(redcliffeGraphine).
//    enablePlugins(play.PlayScala)

}
