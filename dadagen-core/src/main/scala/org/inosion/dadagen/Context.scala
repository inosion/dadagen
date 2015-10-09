package org.inosion.dadagen

/**
 * Context object used "during execution"
 * It is used to keep context between field and "collection" execution
 *
 * The should be turned to a State Monad
 */
class Context(currentRow:Int) {

  import scala.collection.mutable.{HashMap => MHashMap}

  val dataFieldState: MHashMap[String, Any] = MHashMap.empty

  override def toString = s"Ctx($currentRow - ${dataFieldState})"

}
