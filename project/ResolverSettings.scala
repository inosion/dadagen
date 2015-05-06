import sbt._

/**
 * Resolvers
 */
object ResolverSettings {

  /**
   * The local / development resolvers: it includes the default ones + Scala Linter.
   */
  lazy val resolvers = Seq(
    Resolver.defaultLocal,
    Resolver.mavenLocal,
    Resolver.typesafeRepo("snapshots"),
    Resolver.typesafeRepo("releases"),
    Resolver.sonatypeRepo("releases"),
    Resolver.sonatypeRepo("snapshots"),
    // "Linter" at "http://hairyfotr.github.io/linteRepo/releases",
    // "krasserm" at "http://dl.bintray.com/krasserm/maven",
    "spray repo" at "http://repo.spray.io/",
    "spray nightlies" at "http://nightlies.spray.io",
    "twitter repo" at "http://maven.twttr.com"                               ,
    "Journalio Repo" at "http://repo.eligotech.com/nexus/content/repositories/eligosource-releases"
  )

}
