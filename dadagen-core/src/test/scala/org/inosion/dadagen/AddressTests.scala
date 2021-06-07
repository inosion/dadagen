package org.inosion.dadagen

import org.inosion.dadagen.generators.{FullAddress, AddressGenerator}
import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

/**
 * @author rbuckland
 */
@RunWith(classOf[JUnitRunner])
class AddressTests extends AnyFlatSpec with Matchers {

  "generating an address in full" should "look good :-)" in {

    implicit val rand = new MyRandom()

    val ramondom = DadagenStringList(List(
      AddressGenerator("fulllineaddress",FullAddress)
    ))

    val result = ramondom.generateAll(101)

    // TODO - write a regexp to show that address style addresses come out.

    // for now, just see we have 100
    result.length should be (101)


  }
}


class MyRandom extends java.util.Random {
  
  println("using myRandom")

}