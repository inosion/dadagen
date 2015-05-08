package org.inosion.dadagen.support.gatling

import java.util.Date

import akka.actor.{Props, ActorRef}
import io.gatling.core.Predef._
import io.gatling.core.result.message.Status
import io.gatling.http.Predef._
import io.gatling.core.action.{Chainable, Failable}
import io.gatling.core.action.builder.ActionBuilder
import io.gatling.core.config.Protocols
import io.gatling.core.feeder.FeederWrapper
import io.gatling.core.result.writer.{DataWriterClient, DataWriter}
import io.gatling.core.validation.{Success, Failure, Validation}
import io.gatling.jms.MessageReceived

class DoNothingAction(val next: ActorRef) extends Chainable with Failable with DataWriterClient{

  override def executeOrFail(session: Session): Validation[_] = {
      val now = new Date().getTime
      session.attributes.get("message") match {
        case None => Failure("missing the message from the dadagen")
        case Some(msg:String) => {
          writeRequestData(session, session.scenarioName, now, now + 1, now + 3, now + 4, Status("OK"), Some(msg))
          Success(msg)
        }
      }
  }

}

class ExampleGatlingSimulation extends Simulation {

  def doNothing() = new ActionBuilder {
    override def build(next: ActorRef, protocols: Protocols) = system.actorOf(Props(new DoNothingAction(next)))
  }

  import org.inosion.dadagen.api.scaladsl._

  val feeder = dadagen asMaps {
     field { "id".rownumber }.
     field { "gender".gender }.
     field { "firstname".name firstname }.
     field { "surname".name surname }.
     // Combine all the values together .. order (what it depends on) does not matter
     field { "message".template("${id} - ${firstname} ${surname} (${gender}) i:${int} ${ref}")}.
     field { "int".number between 10 and 99876 }.
     field { "ref".regexgen("[a-f]{6}-[0-9a-f]{8}") }
  } generate() // call generate to make the Iterator

  val scn = scenario("Actor Gets a Message").feed(feeder).exec(doNothing)

  setUp(scn.inject(atOnceUsers(10)))

}
