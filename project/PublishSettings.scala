import sbt._
import Keys._

/**
 * This object includes the publishing mechanism. We simply publish to the ``artifacts`` directory,
 * which Jenkins build uses to automatically push the built artefacts to Artifactory.
 */
object PublishSettings {

  lazy val publishSettings: Seq[Def.Setting[_]] = Seq(
    publishArtifact in (Compile, packageDoc) := false,
    publishMavenStyle := true,
    pomIncludeRepository := { _ => false },
    publishTo := {
      val nexus = "https://nexus.straight.io/"
      if (isSnapshot.value)
        Some("snapshots" at nexus + "content/repositories/snapshots")
      else
        Some("releases"  at nexus + "service/local/staging/deploy/maven2")
    },
    pomExtra := (
        <url>https://github.com/inosion/dadagen</url>
        <licenses>
          <license>
            <name>Apache 2.0 (c) Inosion 2015</name>
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
        </developers>),
    credentials ++= (for {
      username <- Option(System.getenv().get("SONATYPE_USERNAME"))
      password <- Option(System.getenv().get("SONATYPE_PASSWORD"))
    } yield Credentials("Sonatype Nexus Repository Manager", "oss.sonatype.org", username, password)).toSeq
  )

}
