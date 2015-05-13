import sbt._

/**
 * Our Project Dependencies
 */
object Dependencies {

  import BaseSettings.Version

  def compile   (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "compile")
  def provided  (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "provided")
  def test      (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "test")
  def runtime   (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "runtime")
  def container (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "container")

  // we use this for the case class copier method
  object scalalang {
    val reflect    = "org.scala-lang"     % "scala-reflect"                  % Version.Scala
    val compiler    = "org.scala-lang"     % "scala-compiler"                 % Version.Scala
  }

  // for inspecting Beans and the like
  object commons { 
    val beanutils  = "commons-beanutils" % "commons-beanutils" % "1.8.3"
  }

  // for generating regexpression strings
  val generex = "com.github.mifmif" % "generex" % "0.0.2"

  // excellent config support
  val typesafeconfig = "com.typesafe" % "config" % "1.2.1"

  // opencsv wrapper
  // "com.bizo:mighty-csv_2.11:0.2"

  object jackson { 
    val version = "2.4.0"
    val dfcsv = "com.fasterxml.jackson.dataformat" % "jackson-dataformat-csv" % version
  }

  /*
   * Testing and Test helpful Libs
   */
  object gatling {
    val version = "2.1.5"
    val testFramework = "io.gatling"            % "gatling-test-framework" % version
    val highcharts    = "io.gatling.highcharts" % "gatling-charts-highcharts" % version
  }

  object jmeter { 
    val version = "2.13"
    val core = "org.apache.jmeter" % "ApacheJMeter_core" % version notTransitive()
    // jmeter has dependencies on the older non org.apache.commons. groupId/ commons-pool2#commons-pool2;2.3
    // so we will explicitly bring them in
//    val commonsMath3 = "commons-math3" % "commons-math3" % "3.4.1" from "http://central.maven.org/maven2/org/apache/commons/commons-math3/3.4.1/commons-math3-3.4.1.jar"
///    val commonsPool2 = "commons-pool2" % "commons-pool2" % "2.3" from "http://central.maven.org/maven2/org/apache/commons/commons-pool2/2.3/commons-pool2-2.3.jar"
  }


  val akka = "com.typesafe.akka" %% "akka-actor" % "2.3.9"

  val scalastm = "org.scala-stm" %% "scala-stm" % "0.7"

  // for the Groovy API for others
  val groovy = "org.codehaus.groovy" % "groovy-all" % "2.4"

  //
  // Logging
  //

  object logging {
    val logback        = "ch.qos.logback"      % "logback-classic"               % "1.0.13"
    val slf4jJcl       = "org.slf4j"           % "jcl-over-slf4j"                % "1.7.6"
    val slf4j          = "org.slf4j"           % "slf4j-api"                     % "1.7.6"
  }

  //
  // Test Dependencies
  //
  object testing {
    val scalatest       = "org.scalatest"      %% "scalatest"    % "2.2.2"
    val scalacheck      = "org.scalacheck"     %% "scalacheck"   % "1.12.1"
    val scalamock       = "org.scalamock"      %% "scalamock-scalatest-support" % "3.2"
  }

}
