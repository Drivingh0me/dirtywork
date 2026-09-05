# All AI use is explicitely disclosed in the source code
Sections using AI are marked off.
eg.
```
//===================
// AI assisted.
let x = 0;
//===================
```

## Categories of AI usage
* Generated - An LLM has directly written the code, unsupervised or
minimally supervised. This is used sparingly.
* Assisted - AI assisted a human write the code, likely filled in
gaps in understanding of tools or features.
* Debugged - A human used AI to assist debugging.

## Validations of AI code
* unvalidated - A human has not confirmed the code section is valid.
* validated - A human has read the code, understands all aspects of it's
execution and has found it to be sound. Some minor changes to the code
by a human were likely made.
