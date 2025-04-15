export type ZkUserVerification = {
  "version": "0.1.0",
  "name": "zk_user_verification",
  "instructions": [
    {
      "name": "initializeMainInfo",
      "accounts": [
        {
          "name": "mainInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "accessType",
          "type": "string"
        },
        {
          "name": "fileName",
          "type": "string"
        }
      ]
    },
    {
      "name": "setAllowedTimeRange",
      "accounts": [
        {
          "name": "mainInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "startHour",
          "type": "u8"
        },
        {
          "name": "endHour",
          "type": "u8"
        }
      ]
    },
    {
      "name": "verifyUser",
      "accounts": [
        {
          "name": "mainInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "proof",
          "type": {
            "defined": "UserProof"
          }
        },
        {
          "name": "accessType",
          "type": "string"
        },
        {
          "name": "fileName",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "mainInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "accessType",
            "type": "string"
          },
          {
            "name": "fileName",
            "type": "string"
          },
          {
            "name": "startHour",
            "type": "u8"
          },
          {
            "name": "endHour",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "UserProof",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proofA",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "proofB",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          },
          {
            "name": "proofC",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "publicSignals",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidFileName",
      "msg": "Invalid file name."
    },
    {
      "code": 6001,
      "name": "InvalidAccessType",
      "msg": "Invalid access type."
    },
    {
      "code": 6002,
      "name": "InvalidTimeRange",
      "msg": "Invalid time range."
    },
    {
      "code": 6003,
      "name": "InvalidTime",
      "msg": "Invalid time for verification."
    },
    {
      "code": 6004,
      "name": "VerificationFailed",
      "msg": "Verification failed."
    },
    {
      "code": 6005,
      "name": "Unauthorized",
      "msg": "Unauthorized access."
    }
  ]
};

export const IDL: ZkUserVerification = {
  "version": "0.1.0",
  "name": "zk_user_verification",
  "instructions": [
    {
      "name": "initializeMainInfo",
      "accounts": [
        {
          "name": "mainInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "accessType",
          "type": "string"
        },
        {
          "name": "fileName",
          "type": "string"
        }
      ]
    },
    {
      "name": "setAllowedTimeRange",
      "accounts": [
        {
          "name": "mainInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "startHour",
          "type": "u8"
        },
        {
          "name": "endHour",
          "type": "u8"
        }
      ]
    },
    {
      "name": "verifyUser",
      "accounts": [
        {
          "name": "mainInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "user",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "proof",
          "type": {
            "defined": "UserProof"
          }
        },
        {
          "name": "accessType",
          "type": "string"
        },
        {
          "name": "fileName",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "mainInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "accessType",
            "type": "string"
          },
          {
            "name": "fileName",
            "type": "string"
          },
          {
            "name": "startHour",
            "type": "u8"
          },
          {
            "name": "endHour",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "UserProof",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proofA",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "proofB",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          },
          {
            "name": "proofC",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          },
          {
            "name": "publicSignals",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidFileName",
      "msg": "Invalid file name."
    },
    {
      "code": 6001,
      "name": "InvalidAccessType",
      "msg": "Invalid access type."
    },
    {
      "code": 6002,
      "name": "InvalidTimeRange",
      "msg": "Invalid time range."
    },
    {
      "code": 6003,
      "name": "InvalidTime",
      "msg": "Invalid time for verification."
    },
    {
      "code": 6004,
      "name": "VerificationFailed",
      "msg": "Verification failed."
    },
    {
      "code": 6005,
      "name": "Unauthorized",
      "msg": "Unauthorized access."
    }
  ]
};
