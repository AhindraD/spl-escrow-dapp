/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/splescrowdapp.json`.
 */
export type Splescrowdapp = {
  "address": "coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF",
  "metadata": {
    "name": "splescrowdapp",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [],
  "errors": [
    {
      "code": 6000,
      "name": "dataFieldEmpty",
      "msg": "Data field Cannot Be Empty"
    }
  ],
  "constants": [
    {
      "name": "anchorDiscriminator",
      "type": "u8",
      "value": "8"
    }
  ]
};
