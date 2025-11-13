// src/utils/headers.js

export function generateHeaders(token, options = {}) {
  const headers = {
    accept: options.accept || "*/*",
    "accept-language": options.acceptLanguage || "en,en-US;q=0.9,ar;q=0.8",
    authorization: token.trim(),
    priority: "u=1, i",
    "sec-ch-ua":
      '"Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141"',
    "User-Agent":
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36",
    "sec-ch-ua-mobile": "?0",
    "sec-ch-ua-platform": '"Linux"',
    "sec-fetch-dest": "empty",
    "sec-fetch-mode": "cors",
    "sec-fetch-site": "same-origin",
    "x-discord-locale": "en-US",
    "x-super-properties": generateSuperProperties(),
    "content-type": "application/json",
    ...options.extraHeaders,
  };

  if (options.superProperties) {
    headers["x-super-properties"] = options.superProperties;
  }

  return headers;
}

function generateSuperProperties() {
  const superProperties = {
    os: "Linux",
    browser: "Chrome",
    device: "",
    system_locale: "en-US",
    browser_user_agent:
      "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36",
    browser_version: "124.0.0.0",
    os_version: "10",
    referrer: "",
    referring_domain: "",
    referrer_current: "",
    referring_domain_current: "",
    release_channel: "stable",
    client_build_number: 9298544,
    client_event_source: null,
    design_id: 0,
  };

  return Buffer.from(JSON.stringify(superProperties)).toString("base64");
}
