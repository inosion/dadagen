package org.inosion.dadagen


import org.inosion.dadagen.generators.DataGenerator

/* reducing compiled class sizes in the releases - http://blog.duh.org/2011/11/scala-pitfalls-trait-bloat.html */
abstract class AbstractIterator[+A] extends Iterator[A]

abstract class IteratingRandomDataGenerator[B] extends RandomDataGenerator[Iterator[B]]

abstract class RandomDataGenerator[A] {

  def generators: List[DataGenerator[_]]

  def generate(): A

  /**
   * Return a list of "nodes" which are depended upon
   */
  val dependentOn: List[String] = {
    require(generators != null, "A list of Generators is required")
    generators.map {
      _.dependencies
    }.flatten.distinct
  }

  private[this] val dependencyNodeList = generators.map(x => SimpleNode(x.name, x.dependencies))

  // internal class used for creating named dependency graphs
  // a name of the generator and what other named generator's it is also dependant on
  private[this] case class SimpleNode(name: String, deps: List[String])


  /**
   * This is NOT the most efficient way to do this, but for our small runs .. it works ok for now.
   * @return
   */
  private[dadagen] val dependencyOrdered: List[DataGenerator[_]] = {
    topoSort(dependencyNodeList).map(s => generators.find( g => g.name.equals(s) ).get)
  }


  // if there is no dependencies, we can return the order of the generators as is
  private[dadagen] val generatorsOrder = dependentOn.isEmpty match {
    case true => generators
    case false => dependencyOrdered
  }
  /**
   *
   * The original generator order, (the order of "generators" defined) needs to be preserved.
   * After "generating all the values in a list, we would need to reorder that list back in the "generator" order.
   * This index can be
   * if the columns depend on each other they will get all mixed up
   * so we need to reorder the result to match the original incoming order
   *
   * This magic below does this task.
   *
   * The original order is (c,a,b)
   * the "generating order" is (b,a,c)  --> because a is dependant on b, and/or, c is dependant on a and/or b
   * so the results will be created in b,a,c but need to be returned as c,a,b order
   *
   * (b,a,c) to (  (b,0),  (a,1),  (c,2)  )
   * to (  (a,1),  (b,0),  (c,2)  ) // sorted on name
   * zip the original  ( (c,0), (a,1) (b,2) )
   * and sort ( (a,1) (b,2), (c,0) ) on name
   *
   * now zip both (  ((a,1),(a,1)),  ((b,0),(b,2)),  ((c,2),(c,0))  )
   * now sort on first index and extract 2nd index // that sorted is the executing order
   * (  ((b,0),(b,2)),  ((a,1),(a,1)),  ((c,2),(c,0))  ) --> (2,1,0)
   *
   * (b,2)  (a,1)  (c,0)  ->  (c,a,b)
   *
   */
  val dependencySortIndex = dependentOn.isEmpty match {
    case true => null
    case false => dependencyOrdered.zipWithIndex.sortBy(_._1.name).zip(
      generators.zipWithIndex.sortBy(_._1.name)
    ).sortBy(_._1._2).map(_._2._2)
  }


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
        if (g.deps.isEmpty) g.name :: topoSort(gs.diff(List(g)).map(x => SimpleNode(x.name, x.deps.diff(List(g.name)))))
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

  import scala.collection.mutable.{ HashMap => MHashMap }

  val dataFieldState: MHashMap[String, Any] = MHashMap.empty

  override def toString = dataFieldState.toString()

}



