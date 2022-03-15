# A Simple Discuss

Use Rocket rc-0.5 , SQLite

- [x] Add a Topic
- [x] Update Topic Father Node
- [x] Get Topic by id
- [x] Get Topic by string eg:1,2,3
- [ ] Set @xxx 
- [ ] Use MySQL instead,and get id by inserting result
- [ ] Remove Test routes and modules
- [ ] Discuss by URL eg: http://dis.com/url/https%3A%2F%2Fwww.ftls.xyz%2Fposts%2Frust-rocket3%2F
- [ ] Build Another Scheme And Convert URL into Topic ID
- [ ] Build Web-Front Page
- [ ] Add a WebSite Main Page

# API Documentation

1. Jump To Discussion By URL

class: GET

eg: http://dis.com/url/https%3A%2F%2Fwww.ftls.xyz%2Fposts%2Frust-rocket3%2F to it's Discussion Page

fn: get_url_discussion

2. Post a new discussion from Front

class: POST

fn: post_discussion

3. Get a discussion by id And Show it's child node content

class: GET

fn: get_discussion

4. Get a discussion by id

class: GET

fn: get_single_discussion

5. Get Discussions by String like "1,2,3"

class: GET

fn: get_discussion_many

