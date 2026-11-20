---
path: Documents/Mail/Re Fare Payments peak forecast for 28 April - Louise.eml
kind: text
author: Louise Turner
derived_from:
---
From: Louise Turner <louise@palmer-capacity.example>
To: Daniel Jennings <daniel@brt-booking.example>
Cc: Matthew Pickering <matthew@brt-booking.example>
Date: Mon, 28 Apr 2025 09:12:00 +0100
Subject: Re: Fare Payments peak forecast for 28 April
Message-ID: <f4385ae7-ec96-4475-b5b0-c37ac5d9e142@palmer-capacity.example>
In-Reply-To: <9c61c43b-5913-4d75-8f9e-2df3cb77ea06@brt-booking.example>
References: <7b28c5e4-0b63-4b59-93ce-d0f641a8de72@palmer-capacity.example> <9c61c43b-5913-4d75-8f9e-2df3cb77ea06@brt-booking.example>

Hi Daniel,

Yes. In PCS-DF-20250425-001 each UTC hour includes its starting boundary and excludes its ending boundary, so 18:00 belongs to the 18:00–19:00 band.

The denominator is the same 1,000 eligible attempts per minute you cite. Duplicate retry indications are excluded from the forecast numerator, as is diagnostic traffic; please do not add those indications back when comparing it with that baseline.

The 2.30× forecast is demand alone. It does not include any reserve BRT elects to hold above it.

Best,
Louise
