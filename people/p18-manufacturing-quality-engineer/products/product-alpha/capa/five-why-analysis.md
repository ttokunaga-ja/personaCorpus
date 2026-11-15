# Five-Why Analysis: NCR-026

**Product:** Product Alpha  
**Organization:** ForgeLine  
**Review date:** 2026-08-11

## Problem statement

Product Alpha locator readings exceeded the approved +/-0.18 mm tolerance during final verification.

## Causal chain

1. Why did readings exceed tolerance? The locating feature shifted during repeated fixture cycles.
2. Why did it shift? The locator sleeve showed wear beyond its useful fit range.
3. Why was wear not detected earlier? The setup confirmation checked position but not sleeve condition.
4. Why was sleeve condition omitted? The work instruction lacked a defined replacement trigger.
5. Why was no trigger defined? Prior control planning treated the sleeve as a durable fixture element.

## Root cause and response

ForgeLine will control sleeve condition as a replaceable fixture component, verify setup against +/-0.18 mm, and train inspection cells on the revised visual check. The effectiveness review covers the next three released Product Alpha lots.
