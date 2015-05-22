package org.inosion.dadagen

import org.inosion.dadagen.generators.{FullAddress, AddressGenerator}
import org.scalatest.{Matchers, FlatSpec}

/**
 * @author rbuckland
 */
class AddressTests  extends FlatSpec with Matchers {

  "generating an address in full" should "look good :-)" in {

    implicit val rand = new MyRandom()

    val ramondom = ListOfStringsGenerator(List(
      AddressGenerator("fulllineaddress",FullAddress)
    ))

    val result = ramondom.generateAll(100)

    // TODO - write a regexp to show that address style addresses come out.

    // for now, just see we have 100
    result.length should be (100)


  }
}


class MyRandom extends java.util.Random {
  
  println("using myRandom")

}