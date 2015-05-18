import sbt._
import Keys._
import bintray.BintrayKeys._

/**
 * This object includes the publishing mechanism. We simply publish to the ``artifacts`` directory,
 * which Jenkins build uses to automatically push the built artefacts to Artifactory.
 */
object PublishSettings {

  lazy val publishSettings: Seq[Def.Setting[_]] = Seq(
    //publishArtifact in (Compile, packageDoc) := false,
    //bintrayReleaseOnPublish in ThisBuild := false,
    publishMavenStyle := true,
    //pomIncludeRepository := { _ => false },
    licenses += ("APSL-2.0", url("http://opensource.org/licenses/Apache-2.0")),
    pomExtra := (
        <url>https://github.com/inosion/dadagen</url>
        <licenses>
          <license>
            <name>Apache 2.0 (c) 2015 Inosion</name>
          </license>
        </licenses>
        <scm>
          <url>git@github.com:inosion/dadagen.git</url>
          <connection>scm:git:git@github.com:inosion/dadagen.git</connection>
        </scm>
        <developers>
          <developer>
            <id>rbuckland</id>
            <name>Ramon Buckland</name>
            <url>http://inosion.com</url>
          </developer>
        </developers>
      )
  )

}
