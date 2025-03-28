/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/moon_wallet_program.json`.
 */
export type MoonWalletProgram = {
  "address": "FVmLk6UEG6YJAhDmUgGGPCNuzs1L1ipha6SYgncrEFUC",
  "metadata": {
    "name": "moonWalletProgram",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addGuardian",
      "discriminator": [
        167,
        189,
        170,
        27,
        74,
        240,
        201,
        241
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "guardian",
          "writable": true
        },
        {
          "name": "guardianPubkey"
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "guardianPubkey",
          "type": "pubkey"
        },
        {
          "name": "guardianName",
          "type": "string"
        },
        {
          "name": "recoveryHashIntermediate",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "configureWebauthn",
      "discriminator": [
        40,
        149,
        116,
        224,
        148,
        48,
        159,
        54
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "owner",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "webauthnPubkey",
          "type": {
            "array": [
              "u8",
              33
            ]
          }
        }
      ]
    },
    {
      "name": "initializeMultisig",
      "discriminator": [
        220,
        130,
        117,
        21,
        27,
        227,
        78,
        213
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "owner"
        },
        {
          "name": "feePayer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "threshold",
          "type": "u8"
        },
        {
          "name": "recoveryHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "credentialId",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "recoverAccess",
      "discriminator": [
        226,
        22,
        59,
        155,
        84,
        251,
        194,
        9
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "newOwner"
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "recoveryHashIntermediate",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "newWebauthnPubkey",
          "type": {
            "array": [
              "u8",
              33
            ]
          }
        }
      ]
    },
    {
      "name": "recoverAccessByGuardian",
      "discriminator": [
        210,
        31,
        244,
        215,
        121,
        93,
        165,
        99
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "guardian"
        },
        {
          "name": "guardianPubkey"
        },
        {
          "name": "newOwner"
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "recoveryHashIntermediate",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "newWebauthnPubkey",
          "type": {
            "array": [
              "u8",
              33
            ]
          }
        }
      ]
    },
    {
      "name": "removeGuardian",
      "discriminator": [
        72,
        117,
        160,
        244,
        155,
        185,
        71,
        18
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "guardian",
          "writable": true
        },
        {
          "name": "guardianPubkey"
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": []
    },
    {
      "name": "storePasswordHash",
      "discriminator": [
        242,
        169,
        229,
        238,
        249,
        138,
        212,
        106
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "security",
          "writable": true
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "passwordHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "storeRecoveryHash",
      "discriminator": [
        188,
        226,
        179,
        52,
        171,
        198,
        28,
        159
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "owner",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "recoveryHashIntermediate",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "recoverySalt",
          "type": {
            "array": [
              "u8",
              16
            ]
          }
        }
      ]
    },
    {
      "name": "updateGuardianStatus",
      "discriminator": [
        17,
        169,
        132,
        234,
        235,
        231,
        211,
        79
      ],
      "accounts": [
        {
          "name": "multisig"
        },
        {
          "name": "guardian",
          "writable": true
        },
        {
          "name": "guardianPubkey"
        },
        {
          "name": "owner",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "isActive",
          "type": "bool"
        }
      ]
    },
    {
      "name": "verifyAndExecute",
      "discriminator": [
        37,
        165,
        237,
        189,
        225,
        188,
        58,
        41
      ],
      "accounts": [
        {
          "name": "multisig",
          "writable": true
        },
        {
          "name": "clock"
        },
        {
          "name": "instructionSysvar"
        },
        {
          "name": "systemProgram"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "destination",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "action",
          "type": "string"
        },
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "actionParams"
            }
          }
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "timestamp",
          "type": "i64"
        },
        {
          "name": "message",
          "type": "bytes"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "guardian",
      "discriminator": [
        57,
        234,
        122,
        214,
        12,
        246,
        9,
        45
      ]
    },
    {
      "name": "multiSigWallet",
      "discriminator": [
        93,
        17,
        107,
        133,
        10,
        77,
        189,
        238
      ]
    },
    {
      "name": "security",
      "discriminator": [
        42,
        116,
        80,
        35,
        124,
        17,
        68,
        246
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidOperation",
      "msg": "Không có quyền hoặc dữ liệu không hợp lệ"
    },
    {
      "code": 6001,
      "name": "limitExceeded",
      "msg": "Giới hạn đã đạt tối đa"
    },
    {
      "code": 6002,
      "name": "guardianError",
      "msg": "Guardian không hợp lệ"
    },
    {
      "code": 6003,
      "name": "invalidConfig",
      "msg": "Cấu hình không hợp lệ"
    },
    {
      "code": 6004,
      "name": "invalidRecovery",
      "msg": "Recovery không hợp lệ"
    },
    {
      "code": 6005,
      "name": "invalidThreshold",
      "msg": "Ngưỡng không hợp lệ"
    },
    {
      "code": 6006,
      "name": "webAuthnNotConfigured",
      "msg": "WebAuthn chưa được cấu hình"
    },
    {
      "code": 6007,
      "name": "nameTooLong",
      "msg": "Tên ví không được vượt quá 32 ký tự"
    },
    {
      "code": 6008,
      "name": "invalidRecoveryKey",
      "msg": "Recovery key không hợp lệ"
    },
    {
      "code": 6009,
      "name": "noGuardians",
      "msg": "Không có guardian nào để xóa"
    },
    {
      "code": 6010,
      "name": "invalidNonce",
      "msg": "Nonce không hợp lệ"
    },
    {
      "code": 6011,
      "name": "futureTimestamp",
      "msg": "Timestamp thuộc về tương lai"
    },
    {
      "code": 6012,
      "name": "outdatedTimestamp",
      "msg": "Timestamp quá cũ"
    },
    {
      "code": 6013,
      "name": "expiredTimestamp",
      "msg": "Timestamp đã hết hạn"
    },
    {
      "code": 6014,
      "name": "instructionMissing",
      "msg": "Instruction xác thực chữ ký bị thiếu"
    },
    {
      "code": 6015,
      "name": "invalidSignatureVerification",
      "msg": "Xác thực chữ ký không hợp lệ"
    },
    {
      "code": 6016,
      "name": "publicKeyMismatch",
      "msg": "Public key không khớp với wallet"
    },
    {
      "code": 6017,
      "name": "messageMismatch",
      "msg": "Message không khớp"
    },
    {
      "code": 6018,
      "name": "invalidInstructionData",
      "msg": "Dữ liệu instruction không hợp lệ"
    },
    {
      "code": 6019,
      "name": "invalidSignatureCount",
      "msg": "Số lượng chữ ký không hợp lệ"
    },
    {
      "code": 6020,
      "name": "unsupportedAction",
      "msg": "Hành động không được hỗ trợ"
    },
    {
      "code": 6021,
      "name": "invalidGuardian",
      "msg": "Guardian không hợp lệ hoặc không được tìm thấy"
    },
    {
      "code": 6022,
      "name": "inactiveGuardian",
      "msg": "Guardian đang không hoạt động"
    }
  ],
  "types": [
    {
      "name": "actionParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "destination",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "tokenMint",
            "type": {
              "option": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "guardian",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "pubkey",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "recoveryHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "multiSigWallet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "hasWebauthn",
            "type": "bool"
          },
          {
            "name": "webauthnPubkey",
            "type": {
              "array": [
                "u8",
                33
              ]
            }
          },
          {
            "name": "credentialId",
            "type": "bytes"
          },
          {
            "name": "guardianCount",
            "type": "u8"
          },
          {
            "name": "recoveryHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "recoverySalt",
            "type": {
              "array": [
                "u8",
                16
              ]
            }
          },
          {
            "name": "recoveryNonce",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "transactionNonce",
            "type": "u64"
          },
          {
            "name": "lastTransactionTimestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "security",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "passwordHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
