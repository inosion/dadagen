import sbt._
import Keys._

/**
 * Defines settings for the projects:
 *
 */
object BaseSettings {

  object Version {
    val ThisVersion = "2.2.2"
    val Scala = "2.11.6"
  }

  /**
   * Common project settings
   */
  val baseSettings: Seq[Def.Setting[_]] =
    net.virtualvoid.sbt.graph.Plugin.graphSettings ++
      Seq(
        organizationHomepage := Some(new URL("https://inosion.org/datdom")),
        organization := "org.inosion.datdom",
        scalaVersion := Version.Scala,
        version := Version.ThisVersion,
        scalacOptions in Compile ++= Seq(
          "-encoding", "UTF-8",
          "-target:jvm-1.7",
          "-deprecation",
          "-unchecked",
          "-Ywarn-dead-code",
          "-language:postfixOps",
          "-language:implicitConversions",
          "-language:_",
          "-Xlog-reflective-calls",
          "-feature"
        ),
        scalacOptions in (Compile, doc) <++= (name in (Compile, doc), version in (Compile, doc)) map DefaultOptions.scaladoc,
        javacOptions in (Compile, compile) ++= Seq(
          "-source", "1.7",
          "-target", "1.7",
          "-Xlint:unchecked",
          "-Xlint:deprecation",
          "-Xlint:-options"),
        javacOptions in doc := Seq(),
        javaOptions += "-Xmx2G",
        outputStrategy := Some(StdoutOutput),
        // for One-JAR
        exportJars := true,
        fork := true,
        fork in test := true,
        sbtPlugin := false,
        resolvers := ResolverSettings.resolvers
      )

  val projectSettings: Seq[Def.Setting[_]] =
    Seq(
      publishArtifact := true
    )
}
