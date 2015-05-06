package io.straight.radg

import io.straight.radg.randomtypes.DataGenerator

import scala.collection.mutable.{ HashMap => MHashMap }

abstract class RandomDataGenerator[A] {
  def generators: Seq[DataGenerator[_]]
  def generate(): A

  // internal class used for creating named dependency graphs
  case class SimpleNode(name:String,deps:List[String])

  /**
   * This is NOT the most efficient way to do this, but for our small runs .. it works ok for now.
   * @return
   */
  val dependencyOrdered:List[DataGenerator[_]] = {
    topoSort(
      generators.map(x => SimpleNode(x.name, x.dependencies)).toList
    ).map(s => generators.find(_.name.equals(s)).get)
  }

  /**
   * Return a list of "nodes" which are depended upon
   */
  val dependentOn:Seq[String] = generators.map{_.dependencies}.flatten.distinct
  /**
   * The following method derives the orders of each node, by name
   * It would have been better towork out "how" this is working (because it is using shadowing of a var)
   * but it does work.. so move on right now
   *
   * @param gs
   * @return
   */
  def topoSort(gs: List[SimpleNode]): List[String] = {
    gs match {
      case g :: gs => {
        // TODO here instead of returning .name ws should return the "Node" found by that name..
        // As it is, we take the name and reuse that "outside of here" (see dependencyOrdered .. map at the end of the
        // topoSort call. Doing it this way below we need to rebuild in the outer using the name list.
        // the way this sort works is to pass an ever reducing dependency list of SimpleNode's into the depths.
        if (g.deps.isEmpty) g.name :: topoSort( gs.diff(List(g)).map(x => SimpleNode(x.name,x.deps.diff(List(g.name)))))
        else topoSort(gs :+ g)
      }
      case _ => List()
    }
  }
}


/**
 * Context object used "during execution"
 */
class Context(currentRow:Int) {
  val dataFieldState: MHashMap[String, Any] = MHashMap.empty
  override def toString() = dataFieldState.toString()
}


