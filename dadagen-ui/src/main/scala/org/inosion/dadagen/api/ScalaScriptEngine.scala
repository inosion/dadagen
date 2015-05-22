package org.inosion.dadagen.api

import javax.script.{ScriptEngine, ScriptEngineManager}

import org.inosion.dadagen.RandomDataGenerator

import scala.tools.nsc.interpreter.IMain

object ScalaScriptEngine {

  def loadEngine(): ScriptEngine = {
    val engine = new ScriptEngineManager().getEngineByName("scala").asInstanceOf[IMain]

    val cl: ClassLoader = classOf[RandomDataGenerator[_]].getClassLoader
    engine.settings.usejavacp.value = true
    engine

  }

}
