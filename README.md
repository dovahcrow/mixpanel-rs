# Mixpanel library for Rust

It's a [Mixpanel](https://mixpanel.com) data export [api](https://mixpanel.com/docs/api-documentation/data-export-api) v2.0 library for [Rust language](http://www.rust-lang.org)

Currently under heavily development

## features:
* [x] Authentication
* Export
  * [x] export - get a "raw dump" of tracked events over a time period
* Annotations
  * [ ] annotations - list the annotations for a specified date range.
  * [ ] create - create an annotation
  * [ ] update - update an annotation
  * [ ] delete - delete an annotation
* Events
  * [ ] events - get total, unique, or average data for a set of events over a time period
  * [ ] top - get the top events from the last day
  * [ ] names - get the top event names for a time period
* Event Properties
  * [ ] properties - get total, unique, or average data from a single event property
  * [ ] top - get the top properties for an event
  * [ ] values - get the top values for a single event property
* Funnels
  * [ ] funnels - get data for a set of funnels over a time period
  * [ ] list - get a list of the names of all the funnels
* Segmentation
  * [ ] segmentation - get data for an event, segmented and filtered by properties over a time period
  * [ ] numeric - get numeric data, divided up into buckets for an event segmented and filtered by properties over a time period
  * [ ] sum - get the sum of a segment's values per time unit
  * [ ] average - get the average of a segment's values per time unit
  * [ ] Segmentation Expressions - a detailed overview of what a segmentation expression consists of
* Retention
  * [ ] retention - get data about how often people are coming back (cohort analysis)
  * [ ] addiction - get data about how frequently people are performing events
* People Analytics
  * [ ] engage - get data from People Analytics
