# Restful API restful-booker.herokuapp.com

## Overview
This Katalon project is designed to thoroughly evaluate the functionality and reliability of the Restful Booker API. The test suites cover critical aspects of booking management, ensuring the API performs seamlessly under diverse scenarios.

## Test Cases
### 1. PUT Create Booking
Description
This test case updates an existing booking with details fetched from global variables. The booking ID is taken from the global variable bookingid. In the Restful Booker API test suite, API chaining is implemented in the PUT Create Booking test case. This test case updates an existing booking by sending a PUT request to the URL https://restful-booker.herokuapp.com/booking/${bookingid}, where ${bookingid} is a global variable representing the booking ID.

API Endpoint : URL: https://restful-booker.herokuapp.com/booking/${bookingid}

### 2. POST Create Booking
Description
This test case creates a new booking with details from global variables. The newly generated booking ID is stored in the global variable bookingid.

API Endpoint : URL: https://restful-booker.herokuapp.com/booking

### 3. POST Get Token
Description
This test case retrieves an authentication token from https://restful-booker.herokuapp.com/auth and stores it in the global variable token.

API Endpoint : URL: https://restful-booker.herokuapp.com/auth

### 4. GET Booking
Description
This test case fetches booking details by sending a GET request to https://restful-booker.herokuapp.com/booking.

API Endpoint : URL: https://restful-booker.herokuapp.com/booking

### 5. GET BookingIds
Description
This test case retrieves booking IDs by sending a GET request to https://restful-booker.herokuapp.com/booking. Similar to the "GET Booking" test, it focuses on obtaining booking IDs.

API Endpoint : URL: https://restful-booker.herokuapp.com/booking

## Conclusion
Running these test suites will assist in identifying potential issues, ensuring the API's reliability, and contributing to a seamless experience for users interacting with the Restful Booker platform.

Continual testing and refinement of these test cases are encouraged to adapt to any changes in the API and maintain the effectiveness of the testing suite over time. The provided test cases serve as a foundation that can be expanded upon to cover additional scenarios and improve overall test coverage.
