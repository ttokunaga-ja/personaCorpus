import type { BetaEvent } from "./product-beta-contract";
export type Delivery={deliveryId:string;attempt:number;ackUrl:string};export type WebhookEnvelope={eventId:string;occurredAt:string;delivery:Delivery;payload:BetaEvent};export type Ack={eventId:string;accepted:true};export function acknowledge(envelope:WebhookEnvelope):Ack{return{eventId:envelope.eventId,accepted:true};}
