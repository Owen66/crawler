# crawler

To get started with this crawler. Run ```docker-compose up``` from the root of the project.

This crawler supports 3 api.

##Crawl
This will return imediatly with an accepted resonse and then crawl the given domain asyncronously. It will only crawl and index links that contain the given domain. Limiting crawling to links for the given domain keeps the crawling focused on a single domain.

Returns 400 if domain url provided is invalid.

Example curl:
```
curl localhost:8080/crawl -H "content-type:application/json" -d '{"name":"http://www.theregister.com"}'
```

##Count
This will return a count of the links that have been indexed so far for the given domain.

Returns 400 if domain url provided is invalid.

Example curl:
```
curl localhost:8080/count -H "content-type:application/json" -d '{"name":"http://www.theregister.com"}'
```

##List
This will return a list of links that have been indexed so far for the given url.

Returns 400 if domain url provided is invalid.

Example curl:
```
curl localhost:8080/list -H "content-type:application/json" -d '{"name":"http://www.theregister.com"}'
```

Work Needed:

- Still lots of unwraps so more error handling needed
- More tests needed
- More refactoring. Crawler could be broken up a bit.
- Currently crawls until all links are found. This needs work as it might not scale for large sites.
- Api's are not really REST at the moment. Everything is a POST. Count/List should be GET.
- Could improve the consumtion of the frontier by moving to async tasks

