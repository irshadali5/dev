---
title: "Downcasting errors"
description: ""
---


<div class="breadcrumb">

Intermediate › Downcasting errors



# Downcasting errors

Sometimes you want to *recover* from a specific error type. anyhow lets
you "open the box" and inspect what's inside.

## downcast_ref — inspect without consuming


<span class="code-lang">rust`
