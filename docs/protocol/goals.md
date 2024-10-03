# 1 Overview

This text outlines a threat model, a set of desired security properties,
and a high-level system design for Aver.
In ensuring total privacy for communications and
collaboration, Aver helps individuals and teams work more freely and effectively.

This text proposes a system where no sensitive information
(including all documents, and files) is ever stored, seen,
or accessible to anyone except its creator and their chosen collaborators.

This is achieved using end-to-end encryption as
well as additional safeguards, including robust authentication
methods, out-of-band key verification (“mark as verified”),
and two-step authentication.

At a more intuitive level, Aver uses the latest in privacy,
cryptography, and decentralization to keep users’ work and
personal data private and truly owned by each individual.

# 2 Threat Model

Aver’s threat model assumes that adversaries may access any
data sent over a network to or from a client (even data sent
over encrypted network connections). It also assumes that
data stored with a **cloud provider** cannot be assumed to be
confidential.

As a result, we assume the server is **“honest but curious”**!

Aver’s servers will not deliberately deny access to data, and
will not serve compromised client applications over the web,
but that data sent to Aver or stored in Aver databases may be
compromised.

For the most security-conscious users, this second component of the threat model
(maintaining honest client applications) is supported by using subresource integrity and
out-of-band public key verification (see further sections of this document).

Given our mission to build privacy-first software, our threat
model and system design also attempt to protect users from
abuse and discoverability or de-anonymization. This includes
designing server endpoints to prevent user enumeration, and
building mechanisms for users to block others users or remove themselves from (or report) shared content.
Aver also collects absolutely no personally identifying information on user signup.

1. **End-to-end encryption of all sensitive information:**
   The contents and certain metadata (including title, time created, and last modified date)
   associated with every document created or uploaded by a user is only visible to the user
   or shared collaborators.
2. **Resistant to man-in-the-middle attacks:**
   Even without a fully trusted communication channel between the server and clients,
   our model will never reveal document title, contents, and other information.
3. **Resistant to user abuse attacks:**
   As we expand and improve our services, we are particularly sensitive to user abuse as
   a threat vector (as discussed in our threat model).
   One user abuse attack includes sharing unwanted information or documents with a particular target user.
4. **Resistant to impersonation attacks:**
   Our model must be resistant to impersonation of the parties (server or clients).
   It should also inhibit impersonation of other users.
5. **Makes phishing difficult:**
   Aver seeks to prevent adversaries from compromising user accounts
   even if their password is compromised, including using 2FA, and/or hardware tokens.

## 2.1 Open Source

Open-sourcing enables everyone to review, use, and contribute to Aver -
making our community stronger and furthering our mission.

# 3 System design

## 3.1 Overview and encryption protocols

Public-key authenticated encryption allows us to securely and privately share access to end-to-end encrypted
documents in our security model. Under this schema, each user is issued a long-term public signing key and
a medium to long term public key for encryption. Each public key is associated with a corresponding private
key generated using Curve25519. We use **[to be decided]** as our encryption library for both asymmetric
public-key authenticated encryption and secret-key authenticated encryption using xsalsa20-poly1305.

Both algorithms ensure both confidentiality and authenticity of encrypted data (AEAD).

While a user’s encryption and signing public keys can be freely shared and used to securely send or verify
information, all private keys - which can decrypt information or generate signatures - must be kept private.

We discuss how this is done in the following section.

## 3.2 Login, creating accounts, and private key storage

For a new user Alice, our account creation and login system
is designed to:

1. Keep Alice’s private keys safe
2. Ensure sensitive information - such as Alice’s password
   and private keys - are never sent beyond her browser
   window, and
3. Resist brute-force and dictionary attacks.

Account creation occurs according to the following process:

1. Bob enters his email and generates a secure password
   (more than n digits, upper/lower case letters, numbers, and special characters).

   Random encryption and signing keypairs are generated for Bob’s account
   in-browser.
2. On the browser, we run Argon2id to derive a symmetric key from Bob’s password.
   Using HKDF, this symmetric key is used to generate one symmetric key for login
   (using the Secure Remote Password protocol), and another symmetric key for encrypting
   Bob’s secrets (i.e. private keys). This second key is called Bob’s `password_derived_secret`.
3. We use Bob’s `password_derived_secret` to encrypt sensitive data associated with Bob’s account;
   this encrypted data is called Bob’s `encrypted_user_data`.
   This includes Bob’s private keys but not his password.

   This encrypted information is stored by our server but can only be decrypted
   with Bob’s `password_derived_secret`. The next time Bob logs in, Bob downloads
   his `encrypted_user_data` from the server, then decrypts the `encrypted_user_data`
   in-browser by using the `password_derived_secret`. The `password_derived_secret`
   never leaves the browser.

In this system, Bob’s public keys are publicly visible and shareable with other users,
while his private keys are encrypted end-to-end. His password and `password_derived_secret`
are never stored, not even as encrypted data. Bob’s `password_derived_secret` and password
are also never sent over any network, even as encrypted data.

We use the secure remote password (SRP) protocol to authenticate user login.
After Bob is authenticated using SRP, the server sends Bob’s `encrypted_user_data` as well as
a signed JSON Web Token (JWT) to indicate that Bob has properly logged in within a certain amount of time.
In our security model, the time-limited JWT is generally used for “read-only” operations,
including downloading encrypted documents. This is discussed in further detail below.
