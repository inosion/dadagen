package org.inosion.dadagen.api

import javax.script.{ScriptEngineManager, ScriptEngine}
import org.inosion.dadagen.Dadagenerator
import scala.tools.nsc.interpreter.IMain

/**
 * A Scala Object to load up the Java Script Engine
 */
object ScalaScriptEngine {

  val loadEngine: ScriptEngine = {
    val engine = new ScriptEngineManager().getEngineByName("scala").asInstanceOf[IMain]

    val cl: ClassLoader = classOf[Dadagenerator[_]].getClassLoader
    engine.settings.usejavacp.value = true
    engine.settings.feature.value = true
    engine.settings.language.add("postfixOps")
    engine

  }

}
