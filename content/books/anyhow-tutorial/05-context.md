---
title: "Adding context to errors"
description: ""
---


<div class="breadcrumb">

Core concepts › Adding context



# Adding context to errors

This is one of anyhow's most powerful features. Instead of just seeing
*"No such file or directory"*, you can add layers of context so errors
read like a story.

## .context() — add a static string


<span class="code-lang">rust`
